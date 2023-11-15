<script>
    import { closeModal } from 'svelte-modals'
    import { invoke } from '@tauri-apps/api/tauri'
    import { onMount } from 'svelte';

    // provided by Modals
    export let isOpen

    export let partNumber;

    async function onSubmit(e) {
        const formData = new FormData(e.target);

        const data = {};
        for (let field of formData) {
            const [key, value] = field;
            //Check if the value is a number
            if (key === "quantity") {
                data[key] = Number(value);
            }
            else {
                data[key] = value;
            }
        }
        let s = JSON.stringify(data)
        let inpart = s;
        await invoke('print_to_console', {s});
        await invoke('modify_qty', {inpart});
        closeModal()
    }
    async function loadInfo(pn) {
        let qty = await invoke('retrieve_qty', {pn});
        let p = JSON.parse(qty);
        document.getElementById("part_number").value = p.part_number;
        document.getElementById("quantity").value = p.quantity;
    }

    onMount(async () => {
        await loadInfo(partNumber);
    });
</script>

{#if isOpen}
    <div role="dialog" class="modal">
        <div class="contents">
            <h2>Modify a part</h2>
            <form on:submit|preventDefault={onSubmit}>
                <div>
                    <label for="name">Part Number</label>
                    <input type="text" id="part_number" name="part_number" value="" readonly  />
                </div>
                <div>
                    <label for="name">Quantity</label>
                    <input type="number" id="quantity" name="quantity" value=0/>
                </div>
                <button type="submit">Submit</button>
            </form>
        </div>
    </div>
{/if}

<style>
    .modal {
        position: fixed;
        top: 0;
        bottom: 0;
        right: 0;
        left: 0;
        display: flex;
        justify-content: center;
        align-items: center;

        /* allow click-through to backdrop */
        pointer-events: none;
    }
    .contents {
        min-width: 240px;
        border-radius: 6px;
        padding: 16px;
        background: white;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        pointer-events: auto;
    }
    h2 {
        text-align: center;
        font-size: 24px;
    }
    p {
        text-align: center;
        margin-top: 16px;
    }
    .actions {
        margin-top: 32px;
        display: flex;
        justify-content: flex-end;
    }
    form {
        display: flex;
        flex-direction: column;
        width: 300px;
    }

    form > div{
        display: flex;
        justify-content: space-between;
    }

    form > div + * {
        margin-top: 10px;
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
</style>