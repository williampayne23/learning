package main

import (
	"fmt"
	"github.com/urfave/cli/v2"
	"log"
	"os"
    "wl/repl"
)

func main() {
	app := &cli.App{
		Commands: []*cli.Command{
			{
				Name:  "repl",
				Usage: "Run the repl",
				Flags: []cli.Flag{
					&cli.StringFlag{
						Name:  "step",
						Value: "parser",
						Usage: "The step to run",
					},
				},
                Action: func(cCtx *cli.Context) error {
                    fmt.Println("Running repl")
                    var step = cCtx.String("step")
                    if step == "lexer" {
                        fmt.Println("Running lexer")
                        repl.Start(os.Stdin, os.Stdout, repl.Lexer_eval)
                    } else if step == "parser" {
                        fmt.Println("Running parser")
                        repl.Start(os.Stdin, os.Stdout, repl.Parser_eval)
                    }
                    return nil
                },
			},
			{
				Name:    "complete",
				Aliases: []string{"c"},
				Usage:   "complete a task on the list",
				Action: func(cCtx *cli.Context) error {
					fmt.Println("completed task: ", cCtx.Args().First())
					return nil
				},
			},
		},
	}

	if err := app.Run(os.Args); err != nil {
		log.Fatal(err)
	}
}
