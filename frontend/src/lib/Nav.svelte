
<script lang="ts">
    import { push, link } from 'svelte-spa-router';
    import beamLogo from '../assets/Logo.svg'
    import NavButton from './NavButton.svelte';
    export let apps: String[];
</script>

<nav>
    <a href="/" use:link target="_blank" rel="noreferrer">
        <img src={beamLogo} class="logo" alt="Beam Logo" />
    </a>
    <div class="apps">
    {#each apps as app}
        <NavButton href={`/app/${app}`}>{app}</NavButton>
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
}

.apps {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    margin: auto;
    margin-top: 2rem;
    gap: 2em;
}

.logo {
    height: 4em;
    padding: .5em;
}
    
</style>
