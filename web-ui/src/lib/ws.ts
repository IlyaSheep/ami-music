import type { Command } from "../types/commands"
import type { ServerEvent } from "../types/server_event"
import * as library from '$lib/commands/library';
import * as queue from '$lib/commands/queue';
import { daemonState } from "./stores/daemon_states.svelte"
import { writable } from "svelte/store";

let ws: WebSocket | null = null
export const connected = writable(false);

export function connect(url: string) {
  ws = new WebSocket(url)
  ws.onopen = () => {
    connected.set(true)
    library.fetch()
    queue.fetch()
  }
  ws.onclose = () => {
    connected.set(false)
  }
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
