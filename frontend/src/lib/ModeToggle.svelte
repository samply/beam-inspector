<script lang="ts">
    // export let options: string[];
    let selected = "Incoming";
    export let incoming = false;
    $: incoming = selected === "Incoming"
</script>

<div class="radio-group">
    <div class="radio-item">
        <input bind:group={selected} id="radio1" type="radio" value="Incoming">
        <label for="radio1">Incoming</label>
    </div>
    <div class="radio-item">
        <input bind:group={selected} id="radio2" type="radio" value="Outgoing">
        <label for="radio2">Outgoing</label>
    </div>
</div>

<style>
.radio-group {
    display: inline-flex;
    gap: 10px;
    border-radius: 20px;
    background-color: var(--color-gray);
}

.radio-item label {
    display: inline-block;
    padding: 8px 16px;
    border-radius: 20px;
    cursor: pointer;
    /* transition: background-color 0.3s; */
}

.radio-item input[type="radio"] {
    display: none;
}

.radio-item input[type="radio"]:checked + label {
    background-color: var(--color-blue);
    color: #ffffff;
}
</style>
