<script>
import PartTable from "$lib/Parts/PartTable.svelte";
import {openModal} from "svelte-modals";
import NewPartModal from "$lib/Parts/NewPartModal.svelte";
import ModifyPartModal from "$lib/Parts/ModifyPartModal.svelte";

let partTableComponent;
function handleCreatePart() {
    openModal(NewPartModal)
}
function handleModifyPart() {
    if(export_pn !== "") {
        openModal(ModifyPartModal, {partNumber: export_pn})
    }
}

let export_pn = "";
</script>

<div class="headerbar">
    <h1>Parts</h1>
    <button on:click={handleCreatePart}>Create</button>
    <button on:click={handleModifyPart}>Modify</button>
    <button on:click={partTableComponent.fetchPartData}>Refresh Data</button>
</div>
<PartTable bind:selected_pn={export_pn} bind:this={partTableComponent}/>

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
