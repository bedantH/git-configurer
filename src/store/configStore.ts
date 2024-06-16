import { get, writable } from "svelte/store";
import type { IConfig } from "../interface/config";


export let availableConfigs = writable<IConfig[]>(localStorage.getItem("availableConfigs") ? JSON.parse(localStorage.getItem("availableConfigs") || "") : [])
export let activeConfig = writable<IConfig>(localStorage.getItem("activeConfig") ? JSON.parse(localStorage.getItem("activeConfig") || "") : {} as IConfig)

availableConfigs.subscribe((configs) => {
  localStorage.setItem('availableConfigs', JSON.stringify(configs));
});

activeConfig.subscribe((active) => {
  if (active) {
    localStorage.setItem('activeConfig', JSON.stringify(active));
  } else {
    localStorage.removeItem('activeConfig');
  }
});

export function addConfig(config: IConfig) {
  availableConfigs.update((prev) => [...prev, config])
}

export function removeConfig(username: string) {
  availableConfigs.update((prev) => prev.filter((config) => config.username !== username))
}

export function setActiveConfig(username: string) {
  activeConfig.set(get(availableConfigs).find((config) => config.username === username) || {} as IConfig);
}