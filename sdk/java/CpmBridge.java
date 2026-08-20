/*
 * CPM Java SDK — Native Java/JVM Client for CPM / UPM Polyglot Bridge RPC
 *
 * Usage:
 *     CpmBridge bridge = new CpmBridge(null);
 *     Object result = bridge.call("python:math.sqrt", new Object[]{144.0});
 *     System.out.println(result);
 */

package com.cpm.sdk;

import java.io.*;
import java.nio.charset.StandardCharsets;
import java.util.*;

public class CpmBridge {
    private final String cpmBin;

    public CpmBridge(String cpmBin) {
        this.cpmBin = (cpmBin != null && !cpmBin.isEmpty()) ? cpmBin : findCpmBin();
    }

    private static String findCpmBin() {
        File curr = new File(".").getAbsoluteFile();
        for (int i = 0; i < 5; i++) {
            for (String rel : new String[]{"target/release/cpm.exe", "target/debug/cpm.exe", "target/release/cpm", "target/debug/cpm"}) {
                File p = new File(curr, rel);
                if (p.exists()) {
                    return p.getAbsolutePath();
                }
            }
            curr = curr.getParentFile();
            if (curr == null) break;
        }
        return "cpm";
    }

    public Object call(String target, Object[] args) throws Exception {
        StringBuilder argsJson = new StringBuilder("[");
        if (args != null) {
            for (int i = 0; i < args.length; i++) {
                if (args[i] instanceof String) {
                    argsJson.append("\"").append(args[i]).append("\"");
                } else {
                    argsJson.append(args[i]);
                }
                if (i < args.length - 1) argsJson.append(",");
            }
        }
        argsJson.append("]");

        ProcessBuilder pb = new ProcessBuilder(cpmBin, "bridge", "call", target, argsJson.toString());
        Process proc = pb.start();

        BufferedReader reader = new BufferedReader(new InputStreamReader(proc.getInputStream(), StandardCharsets.UTF_8));
        String line;
        StringBuilder output = new StringBuilder();
        boolean capture = false;
        StringBuilder jsonLines = new StringBuilder();

        while ((line = reader.readLine()) != null) {
            output.append(line).append("\n");
            if (line.contains("Response received:")) {
                capture = true;
                continue;
            }
            if (capture && (line.contains("round-trip via stdio RPC") || line.trim().isEmpty())) {
                if (jsonLines.length() > 0) break;
            }
            if (capture) {
                jsonLines.append(line);
            }
        }

        int exitCode = proc.waitFor();
        if (exitCode != 0) {
            throw new RuntimeException("CPM Bridge Call Error: " + output.toString().trim());
        }

        return jsonLines.toString().trim();
    }
}
