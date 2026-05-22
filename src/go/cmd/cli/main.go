package main

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

func main() {
	var rootCmd = &cobra.Command{
		Use:   "cislunar-cli",
		Short: "Cislunar Identity Certificate Authority CLI",
	}

	rootCmd.AddCommand(issueCmd)
	rootCmd.AddCommand(listCmd)
	rootCmd.AddCommand(revokeCmd)
	rootCmd.AddCommand(loginCmd)

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

var issueCmd = &cobra.Command{
	Use:   "issue [subject]",
	Short: "Issue a new certificate",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Printf("Issuing certificate for: %s\n", args[0])
	},
}

var listCmd = &cobra.Command{
	Use:   "list",
	Short: "List certificates",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Listing certificates...")
	},
}

var revokeCmd = &cobra.Command{
	Use:   "revoke [serial]",
	Short: "Revoke a certificate",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Printf("Revoking certificate: %s\n", args[0])
	},
}

var loginCmd = &cobra.Command{
	Use:   "login",
	Short: "Authenticate with the CA",
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Login...")
	},
}
