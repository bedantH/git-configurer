<script lang="ts">
  import { get } from "svelte/store";
  import ConfigDialog from "../components/config-dialog.svelte";
  import type { IConfig } from "../interface/config";
  import { removeConfig, activeConfig, availableConfigs, setActiveConfig } from "../store/configStore";
  import * as RadioGroup from "$lib/components/ui/radio-group/index";
  import Button from "$lib/components/ui/button/button.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import Icon from '@iconify/svelte';

  let allConfigs: IConfig[] = get(availableConfigs)
  let active: IConfig = get(activeConfig)

  availableConfigs.subscribe(value => {
    allConfigs = value;
  });

  activeConfig.subscribe(value => {
    active = value;
  });

  function handleCommandExecution() {
    invoke('run_config_command', {
      username: active.username,
      email: active.email
    })
  }

  function handleDeleteConfig(name: string) {
    removeConfig(name)
  }
</script>

<div class="p-4">
  <div class="flex gap-2 items-center mb-4">
    <img src="/github-mark.svg" class="w-6 h-6" alt="logo" />
    <p class="text-sm font-bold">Github User Configurer</p>
  </div>

  {#key localStorage}
    {#if allConfigs.length === 0}
    <div class="border-[1.1px] border-gray-300 border-dashed rounded-md p-4">
      <p class="text-sm">No Configurations Added</p>
    </div>
    {:else}
      <div class="">
        <RadioGroup.Root
          bind:value={active.username}
          class="flex flex-col gap-2"
          onValueChange={(value) => {
            setActiveConfig(value)
          }}
        >  
          {#each allConfigs as config}
            <div class="flex justify-between items-center border-[1.1px] border-solid border-gray-300 py-3 px-3 rounded-md">
              <div class="flex items-center gap-2">
                <RadioGroup.Item value={config.username} />
                <p class="text-sm">{config.username}</p>
              </div>

              <Button variant="ghost" class="text-md w-fit h-fit py-0 px-0" on:click={() => {
                handleDeleteConfig(config.username)
              }}>
                <Icon icon="mynaui:trash-one" />
              </Button>
            </div>
          {/each}
        </RadioGroup.Root>
      </div>
    {/if}
  {/key}

  <div class="flex gap-2 mt-2">
    <Button on:click={handleCommandExecution}>Save Config</Button>
    <ConfigDialog />
  </div>
</div>