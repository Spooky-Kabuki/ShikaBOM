<script>
    import SvelteTable from "svelte-table";
    import { invoke } from '@tauri-apps/api/tauri'

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
            key: "manufacturer",
            title: "Manufacturer",
            value: v => v.manufacturer,
            sortable: true,
            filterOptions: rows => {
                return Object.values({})
            },
            filterValue: v => v.manufacturer.charAt(0).toLowerCase(),
        },
        {
            key: "package",
            title: "Package",
            value: v => v.package,
            sortable: true,
            filterOptions: rows => {
              return Object.values({});
            },
            filterValue: v => v.package.charAt(0).toLowerCase(),
        },
        {
            key: "label",
            title: "Label",
            value: v => v.label,
            renderValue: v => v.label.charAt(0).toUpperCase() + v.label.substring(1), // capitalize
            sortable: true,
            filterOptions: ["Capacitor", "Resistor"], // provide array
        },
        {
            key: "value",
            title: "Value",
            value: v => v.value,
            renderValue: v => v.value,
            sortable: true,
            filterOptions: {},
        },
        {
            key: "tolerance",
            title: "Tolerance",
            value: v => v.tolerance,
            renderValue: v => v.tolerance !== 0.0 ? v.tolerance : "N/A",
            sortable: true,
            filterOptions: {},
        }
    ];

    export async function fetchPartData() {
        let json_rows = await invoke('fetch_part_data');
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