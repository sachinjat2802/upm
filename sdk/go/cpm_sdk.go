// CPM Go SDK — Native Go Client for CPM / UPM Polyglot Bridge RPC
//
// Usage:
//     import "d:/cpm/sdk/go"
//     bridge := cpm.NewCpmBridge("")
//     res, err := bridge.Call("python:math.sqrt", []interface{}{144.0})

package cpm

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

type CpmBridge struct {
	CpmBin string
}

func NewCpmBridge(cpmBin string) *CpmBridge {
	if cpmBin == "" {
		cpmBin = findCpmBin()
	}
	return &CpmBridge{CpmBin: cpmBin}
}

func findCpmBin() string {
	curr, _ := os.Getwd()
	for i := 0; i < 5; i++ {
		for _, rel := range []string{"target/release/cpm.exe", "target/debug/cpm.exe", "target/release/cpm", "target/debug/cpm"} {
			p := filepath.Join(curr, rel)
			if _, err := os.Stat(p); err == nil {
				return p
			}
		}
		parent := filepath.Dir(curr)
		if parent == curr {
			break
		}
		curr = parent
	}
	return "cpm"
}

func (b *CpmBridge) Call(target string, args []interface{}) (interface{}, error) {
	argsBytes, err := json.Marshal(args)
	if err != nil {
		return nil, err
	}

	cmd := exec.Command(b.CpmBin, "bridge", "call", target, string(argsBytes))
	var outBuf, errBuf bytes.Buffer
	cmd.Stdout = &outBuf
	cmd.Stderr = &errBuf

	if err := cmd.Run(); err != nil {
		return nil, fmt.Errorf("CPM Bridge Call Error: %s", strings.TrimSpace(errBuf.String()))
	}

	stdoutStr := outBuf.String()
	lines := strings.Split(strings.TrimSpace(stdoutStr), "\n")
	capture := false
	var jsonLines []string

	for _, line := range lines {
		if strings.Contains(line, "Response received:") {
			capture = true
			continue
		}
		if capture && (strings.Contains(line, "round-trip via stdio RPC") || strings.TrimSpace(line) == "") {
			if len(jsonLines) > 0 {
				break
			}
		}
		if capture {
			jsonLines = append(jsonLines, line)
		}
	}

	rawJSON := strings.TrimSpace(strings.Join(jsonLines, "\n"))
	if rawJSON != "" {
		var result interface{}
		if err := json.Unmarshal([]byte(rawJSON), &result); err == nil {
			return result, nil
		}
	}

	return strings.TrimSpace(stdoutStr), nil
}
