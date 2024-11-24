package main

import (
	"context"
	"fmt"
	"os"
	"os/signal"

	"github.com/macabu/cronus/internal/account"
	"github.com/macabu/cronus/internal/server"
)

func main() {
	ctx, cancel := signal.NotifyContext(context.Background(), os.Interrupt, os.Kill)
	defer cancel()

	if err := run(ctx); err != nil {
		fmt.Printf("exiting with error: %v", err)
		os.Exit(1)
	}

	fmt.Printf("exiting normally")
}

func run(ctx context.Context) error {
	packetHandler := &account.PacketHandler{}

	srv := server.NewServer(":6900", packetHandler)

	if err := srv.ListenAndServe(ctx); err != nil {
		return fmt.Errorf("listen and serve: %w", err)
	}

	return nil
}
