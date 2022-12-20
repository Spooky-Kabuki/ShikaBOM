<script>
    import Greet from '../lib/Greet.svelte'
    import Part from '../lib/Part.svelte'
    import PartTable from '../lib/PartTable.svelte'

</script>

<div class="headerbar">
    <h1>Parts</h1>
    <button>Create</button>
    <button>Modify</button>
</div>
<PartTable />

<style>
    .headerbar {
        display: inline-block;
    }


</style>