<script>
    import { invoke } from '@tauri-apps/api/tauri'
    let pn = ''
    let mfg = ''

    async function getRow() {
        pn = await invoke('get_pn')
        mfg = await invoke('get_mfg', { pn })
    }
</script>

<div on:load={getRow}>
  <p>Part Number: {pn}</p>
  <p>MFG: {mfg}</p>
</div>
