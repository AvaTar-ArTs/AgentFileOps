// Package agentfileops provides a typed wrapper around the verified afo policy CLI.
package agentfileops

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"os/exec"
)

type Error struct {
	Code    string `json:"code"`
	Message string `json:"message"`
}

func (e *Error) Error() string { return e.Code + ": " + e.Message }

type Client struct{ Binary string }

func (c Client) binary() string {
	if c.Binary == "" {
		return "afo"
	}
	return c.Binary
}

func (c Client) run(ctx context.Context, args ...string) (map[string]any, error) {
	cmd := exec.CommandContext(ctx, c.binary(), args...)
	var stdout, stderr bytes.Buffer
	cmd.Stdout, cmd.Stderr = &stdout, &stderr
	if err := cmd.Run(); err != nil {
		var response Error
		if json.Unmarshal(stderr.Bytes(), &response) == nil && response.Code != "" {
			return nil, &response
		}
		return nil, fmt.Errorf("afo failed: %w: %s", err, stderr.String())
	}
	var response map[string]any
	if err := json.Unmarshal(stdout.Bytes(), &response); err != nil {
		return nil, fmt.Errorf("invalid afo response: %w", err)
	}
	return response, nil
}

func (c Client) NormalizePath(ctx context.Context, base, path string, followSymlinks bool) (map[string]any, error) {
	args := []string{"normalize-path", "--base", base, "--path", path}
	if followSymlinks {
		args = append(args, "--follow-symlinks")
	}
	return c.run(ctx, args...)
}

func (c Client) ClassifyRisk(ctx context.Context, operation string) (map[string]any, error) {
	return c.run(ctx, "classify-risk", "--operation", operation)
}
