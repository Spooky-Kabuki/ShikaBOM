<script>
    import PartTable from '../lib/PartTable.svelte'
    import { Modals, closeModal } from 'svelte-modals'
    import { openModal } from 'svelte-modals'
    import NewPartModal from "$lib/NewPartModal.svelte";
    function handleCreatePart() {
        openModal(NewPartModal)
    }
</script>

<Modals>
    <div
        slot="backdrop"
        class="backdrop"
        on:click={closeModal}>
    </div>
</Modals>

<div class="headerbar">
    <h1>Parts</h1>
    <button on:click={handleCreatePart}>Create</button>
    <button>Modify</button>
</div>
<PartTable />

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
</style>