<script>
    import StorageTable from "$lib/Storage/StorageTable.svelte";
    import {openModal} from "svelte-modals";

    let storageTableComponent;
</script>

<div class="headerbar">
    <h1>Storage</h1>
    <button>Create New Storage Location</button>
    <button>Modify Part Count</button>
    <button on:click={storageTableComponent.fetchStorageData}>Refresh Data</button>
</div>
<StorageTable bind:this={storageTableComponent}/>

<style>
    button {
        background-color: transparent;
        border: none;
        cursor: pointer;
        font-size: 16px;
        padding: 10px;
        transition: background-color 0.3s ease;
        border-radius: 15px;
    }

    button:hover {
        background-color: #ddd;
    }
</style>
