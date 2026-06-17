import type { Command } from "../types/commands"
import type { ServerEvent } from "../types/server_event"
import { daemonState } from "./stores/daemon_states"

let ws: WebSocket | null = null


export function connect(url: string) {
  ws = new WebSocket(url)
  ws.onmessage = (e) => {
    const msg: ServerEvent = JSON.parse(e.data)

    switch (msg.type) {
      case "SendLibrary":
        daemonState.library = msg.data
        break
      case "SendQueue":
        daemonState.queue = msg.data
        break
      case "SendPlayerSnapshot":
        daemonState.player = msg.data
        break
      case "SendPlayerPosition":
        if (daemonState.player) {
          daemonState.player.position = msg.data.position
        }
        break
    }
  }
}

export function send(command: Command) {
  ws?.send(JSON.stringify(command))
}
