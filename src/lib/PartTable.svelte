<script>
    import SvelteTable from "svelte-table";

    let selection = {}

    let rows = [
        { part_number: "08053A331JAT2A", mfg: "AVX", desc: "Cap Ceramic 330pF 25V C0G 5% SMD 0805 125°C Paper T/R",label: "Capacitor", package: "0805", value: "330 pF", tolerance: null },
        { part_number: "1623022-1", mfg: "TE Connectivity / AMP", desc: "Res Thick Film 0603 10K Ohm 5% 1/10W +/- 200ppm/°C Molded SMD SMD", label: "Resistor", package: "0603", value: "330 pF", tolerance: null },
        { part_number: "25SVPF47M", mfg: "Panasonic", desc: "Cap Aluminum Polymer 47uF 25VDC 20%( 6.3 X 5.9mm) SMD 0.03 Ohm 2800mA 5000h 105C T/R", label: "Capacitor", package: "Radial, Can - SMD", value: "47.0 μF", tolerance: 20}

      // etc...
    ];

    // define column configs
    const columns = [
        {
            key: "part_number",
            title: "Part Number",
            value: v => v.part_number,
            sortable: true,
            filterOptions: rows => {
              // generate groupings of 0-10, 10-20 etc...
              let nums = {};
              return Object.values(nums);
            },
            filterValue: v => Math.floor(v.id / 10),
            headerClass: "text-left",
        },
        {
            key: "mfg",
            title: "Manufacturer",
            value: v => v.mfg,
            sortable: true,
            filterOptions: rows => {
                return Object.values({})
            },
            filterValue: v => v.mfg.charAt(0).toLowerCase(),
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
            renderValue: v => v.tolerance != null ? v.tolerance : "N/A",
            sortable: true,
            filterOptions: {},
        }
    ];
</script>

<SvelteTable columns="{columns}" rows="{rows}" selectOnClick={true}></SvelteTable>