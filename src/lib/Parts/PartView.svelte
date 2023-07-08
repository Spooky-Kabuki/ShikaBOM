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
    openModal(ModifyPartModal, {partNumber: export_pn})
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
