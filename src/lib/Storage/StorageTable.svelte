<script>
    import SvelteTable from "svelte-table";
    import { invoke } from '@tauri-apps/api/tauri'

    //TODO: Don't spend much time on the GUI, looks like there's better libraries out there
    let selection = {}
    let selectionSingle = [];
    let rows = [];

    export let selected_pn = selectionSingle.join(", ");
    $: selected_pn = selectionSingle.join(", ");

    // define column configs
    const columns = [
        {
            key: "part_number",
            title: "Part Number",
            value: v => v.part_number,
            sortable: true,
            filterOptions: rows => {
                return Object.values({});
            },
            filterValue: v => Math.floor(v.id / 10),
            headerClass: "text-left",
        },
        {
            key: "location",
            title: "Storage Location",
            value: v => v.location,
            sortable: true,
            filterOptions: rows => {
                return Object.values({})
            },
            filterValue: v => v.location.charAt(0).toLowerCase(),
        },
        {
            key: "quantity",
            title: "Quantity",
            value: v => v.quantity,
            renderValue: v => v.quantity,
            sortable: true,
            filterOptions: {},
        },
        {
            key: "loc_id",
            title: "Location ID",
            value: v => v.loc_id,
            renderValue: v => v.loc_id, // capitalize
            sortable: true,
            filterOptions: {},
        }
    ];

    export async function fetchStorageData() {
        let json_rows = await invoke('fetch_storage_data');
        let s = JSON.stringify(json_rows);
        await invoke('print_to_console', {s});
        rows = JSON.parse(json_rows.toString());
    }
</script>



<pre>Selection: [{selectionSingle.join(", ")}]</pre>
<SvelteTable
        columns="{columns}"
        rows="{rows}"
        selectOnClick="{true}"
        rowKey="part_number"
        bind:selected={selectionSingle}
        selectSingle={true}
        classNameTable="table"
        classNameThead="table-info"
        classNameRowSelected="row-selected"
/>

<style>
    :global(.row-expanded) {
        background-color: #151826;
        color: #fff;
        cursor: crosshair;
    }
    :global(.expanded-content) {
        background-color: #151826;
    }
    :global(.row-selected) {
        background-color: #f8c;
    }
</style>