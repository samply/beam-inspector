<script lang="ts">
    import { push, location } from "svelte-spa-router";

    export let href: string
    $: active = $location === href
</script>

<button class:active on:click={() => push(href)}>
    <slot />
</button>

<style>

button {
  background-color: #f5f5f5;
  border: 2px solid black;
  padding: 10px 28px;
  text-align: center;
  display: inline-block;
  font-size: 16px;
  transition-duration: 0.4s;
  cursor: pointer;
  border-radius: 8px;
}

button:hover {
  border: 2px solid var(--color-blue);
}

.active {
    border: 2px solid var(--color-blue);
}
</style>
