<script>
    import StorageTable from "$lib/Storage/StorageTable.svelte";
    import {openModal} from "svelte-modals";

    let storageTableComponent;
</script>

<div class="headerbar">
    <h1>Storage</h1>
    <button>Create</button>
    <button>Modify</button>
    <button>Refresh Data</button>
</div>
<StorageTable bind:this={storageTableComponent}/>
