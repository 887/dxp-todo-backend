# info

[https://stackoverflow.com/questions/43836861/how-to-run-a-command-in-visual-studio-code-with-launch-json]

Yes, it's possible. Just use:

"type": "node-terminal"

(don't worry it says 'node-terminal', it should work out-of-the-box in VS Code)

and specify your command in "command" property:

"command": "ssh -i xxxx"

[https://stackoverflow.com/questions/53214062/vs-code-launch-multiple-configurations-at-once]

{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "node",
      "request": "launch",
      "name": "Server",
      "program": "${workspaceFolder}/server.js"
    },
    {
      "type": "node",
      "request": "launch",
      "name": "Client",
      "program": "${workspaceFolder}/client.js"
    }
  ],
  "compounds": [
    {
      "name": "Server/Client",
      "configurations": ["Server", "Client"],
      "preLaunchTask": "${defaultBuildTask}",
      "stopAll": true
    }
  ]
}
