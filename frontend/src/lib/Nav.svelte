
<script lang="ts">
    import { push } from 'svelte-spa-router';
    import beamLogo from '../assets/Logo.svg'
    export let apps: String[];
</script>

<nav>
    <a href="/" target="_blank" rel="noreferrer">
        <img src={beamLogo} class="logo" alt="Beam Logo" />
    </a>
    <div class="apps">
    {#each apps as app}
        <button on:click={() => push(`/app/${app}`)}>{app}</button>
    {/each}
    </div>
</nav>


<style>

nav {
    padding: .5rem;
    display: flex;
    flex-direction: column;
    justify-content: center;
    position: sticky;
    top: 0;
    /* align-items: f; */
}

button {
    display: block;
    border: 1px solid gray;
}

.apps {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    margin-left: 3rem;
    gap: 1em;
}

.logo {
    height: 4em;
    padding: .5em;
}
    
</style>
