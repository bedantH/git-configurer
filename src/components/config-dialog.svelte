<script lang="ts">
  import {
    Button,
    buttonVariants
  } from "$lib/components/ui/button/index.js";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { cn } from "$lib/utils";
  import { get } from "svelte/store";
  import { addConfig, availableConfigs, setActiveConfig } from "../store/configStore";

  let username: string = ""
  let email: string = ""

  const handleSubmit = () => {
    addConfig({
      username,
      email
    })

    if (get(availableConfigs).length === 0) {
      setActiveConfig(username);
    }

    username = ""
    email = ""
  }

</script>
 
<Dialog.Root>
  <Dialog.Trigger class={cn(buttonVariants({ variant: "default" }))}
    >Add</Dialog.Trigger
  >
  <Dialog.Content class="p-3 rounded-md max-w-[300px]">
    <div class="flex flex-col justify-start gap-1">
      <div class="items-start gap-4">
        <Label for="username" class="text-xs text-right">Username</Label>
        <Input bind:value={username} id="username" class="text-xs" />
      </div>
      <div class="items-start gap-4">
        <Label for="email" class="text-xs text-right">Email</Label>
        <Input bind:value={email} id="email" class="text-xs" />
      </div>
    </div>
    <Dialog.Footer>
      <Dialog.Close>
        <Button type="submit" on:click={handleSubmit} class="text-xs">Add</Button>
      </Dialog.Close>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>