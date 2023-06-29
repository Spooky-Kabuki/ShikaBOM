<script>
    import PartTable from '../lib/PartTable.svelte'
    import { Modals, closeModal } from 'svelte-modals'
    import { openModal } from 'svelte-modals'
    import NewPartModal from "$lib/NewPartModal.svelte";
	import ModifyPartModal from '$lib/ModifyPartModal.svelte';

    export let activeTab = "parts";

    function switchTab(tab) {
        activeTab = tab;
    }

    function handleCreatePart() {
        openModal(NewPartModal)
    }
    function handleModifyPart() {
        openModal(ModifyPartModal, {partNumber: export_pn})
    }
    let export_pn = "";
</script>

<div class="tab-bar">
    <button class:active={activeTab === 'parts'} on:click={() => switchTab('parts')}>
      Parts
    </button>
    <button class:active={activeTab === 'storage'} on:click={() => switchTab('storage')}>
      Storage
    </button>
    <!-- Add more tabs here -->
</div>

{#if activeTab === 'parts'}
    <div class="headerbar">
        <h1>Parts</h1>
        <button on:click={handleCreatePart}>Create</button>
        <button on:click={handleModifyPart}>Modify</button>
    </div>
    <PartTable bind:selected_pn={export_pn}/>
{:else if activeTab === 'storage'}
    <div class="headerbar">
        <h1>Storage</h1>
    </div>
{:else}
    <div class="headerbar">
        <h1>No tab selected</h1>
    </div>
{/if}



<Modals>
    <div
        slot="backdrop"
        class="backdrop"
        on:click={closeModal}
        on:keypress={closeModal}>
    </div>
</Modals>

<style>
    .headerbar {
        display: inline-block;
    }
    .backdrop {
        position: fixed;
        top: 0;
        bottom: 0;
        right: 0;
        left: 0;
        background: rgba(0, 0, 0, 0.50)
    }
    .tab-bar {
        display: flex;
        justify-content: flex-start;
        align-items: center;
        background-color: #f2f2f2;
        padding: 10px;
    }

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

    .active {
        background-color: #ddd;
    }
</style>