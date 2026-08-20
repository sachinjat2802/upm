// CPM Go Host Process — stdio RPC bridge (upm-bridge/1)
// Implements 4-byte Big-Endian length-prefixed JSON framing over stdio.

package main

import (
	"crypto/sha256"
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"math"
	"os"
)

type MessageEnvelope struct {
	Type       string          `json:"type"`
	ID         string          `json:"id,omitempty"`
	Method     string          `json:"method,omitempty"`
	Args       []interface{}   `json:"args,omitempty"`
	Result     interface{}     `json:"result,omitempty"`
	Error      *UpmError       `json:"error,omitempty"`
}

type UpmError struct {
	ErrorType string `json:"error_type"`
	Message   string `json:"message"`
}

func main() {
	reader := os.Stdin
	writer := os.Stdout

	headerBuf := make([]byte, 4)

	for {
		_, err := io.ReadFull(reader, headerBuf)
		if err != nil {
			if err == io.EOF {
				break
			}
			break
		}

		length := binary.BigEndian.Uint32(headerBuf)
		if length > 64*1024*1024 {
			break
		}

		msgBuf := make([]byte, length)
		_, err = io.ReadFull(reader, msgBuf)
		if err != nil {
			break
		}

		var env MessageEnvelope
		if err := json.Unmarshal(msgBuf, &env); err != nil {
			continue
		}

		if env.Type == "request" {
			resp := handleRequest(env)
			sendResponse(writer, resp)
		} else if env.Type == "ping" {
			sendResponse(writer, MessageEnvelope{Type: "pong", ID: env.ID})
		}
	}
}

func handleRequest(req MessageEnvelope) MessageEnvelope {
	resp := MessageEnvelope{
		Type: "response",
		ID:   req.ID,
	}

	switch req.Method {
	case "ping":
		resp.Result = "pong"
	case "echo":
		if len(req.Args) > 0 {
			resp.Result = req.Args[0]
		} else {
			resp.Result = nil
		}
	case "math.sqrt":
		if len(req.Args) > 0 {
			if num, ok := req.Args[0].(float64); ok {
				resp.Result = math.Sqrt(num)
			} else {
				resp.Error = &UpmError{ErrorType: "TypeError", Message: "Argument must be a number"}
			}
		}
	case "crypto.sha256":
		if len(req.Args) > 0 {
			if str, ok := req.Args[0].(string); ok {
				hash := sha256.Sum256([]byte(str))
				resp.Result = hex.EncodeToString(hash[:])
			}
		}
	case "__inspect__":
		resp.Result = []map[string]interface{}{
			{"name": "math.sqrt", "description": "Go math.Sqrt float64 square root"},
			{"name": "crypto.sha256", "description": "Go crypto/sha256 hashing"},
			{"name": "echo", "description": "Echo input value"},
			{"name": "ping", "description": "Keepalive ping/pong"},
		}
	default:
		resp.Error = &UpmError{ErrorType: "MethodNotFoundError", Message: fmt.Sprintf("Method '%s' not registered on Go host", req.Method)}
	}

	return resp
}

func sendResponse(writer io.Writer, env MessageEnvelope) {
	data, err := json.Marshal(env)
	if err != nil {
		return
	}

	length := uint32(len(data))
	header := make([]byte, 4)
	binary.BigEndian.PutUint32(header, length)

	writer.Write(header)
	writer.Write(data)
}
