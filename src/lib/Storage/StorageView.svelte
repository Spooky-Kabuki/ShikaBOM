<script>
import StorageTable from "$lib/Storage/StorageTable.svelte";
import {openModal} from "svelte-modals";
import ModifyQtyModal from "$lib/Storage/ModifyQtyModal.svelte";

let storageTableComponent;
function handleModifyQty() {
    if(export_pn !== "") {
        openModal(ModifyQtyModal, {partNumber: export_pn});
    }
}

let export_pn = "";
</script>

<div class="headerbar">
    <h1>Storage</h1>
    <button>Create New Storage Location</button>
    <button on:click={handleModifyQty}>Modify Part Count</button>
    <button on:click={storageTableComponent.fetchStorageData}>Refresh Data</button>
</div>
<StorageTable bind:selected_pn={export_pn} bind:this={storageTableComponent}/>

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
