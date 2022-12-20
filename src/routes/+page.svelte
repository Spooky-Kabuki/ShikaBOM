<script>
    import PartTable from '../lib/PartTable.svelte'

    function console_log() {
        console.log("TESTING");
    }

</script>

<div class="headerbar">
    <h1>Parts</h1>
    <button on:click={console_log}>Create</button>
    <button>Modify</button>
</div>
<PartTable />

<style>
    .headerbar {
        display: inline-block;
    }


</style>