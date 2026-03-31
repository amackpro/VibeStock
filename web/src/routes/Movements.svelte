<script>
  /**
   * Movements.svelte — Stock movements history & new movement entry
   *
   * Lets staff record IN / OUT / ADJUSTMENT / RETURN movements.
   * Shows a paginated audit log with color-coded movement type badges.
   */
  import { onMount } from 'svelte';
  import { api }   from '../lib/api.js';
  import { toast } from '../stores/toast.js';
  import { X, Package, Truck, Wrench, Undo2 } from 'lucide-svelte';

  let movements  = [];
  let products   = [];
  let total      = 0;
  let page       = 1;
  let loading    = false;
  let showModal  = false;

  // New movement form
  let form = { product_id: '', movement_type: 'in', quantity: 1, reference: '', notes: '' };

  // ── Data loading ─────────────────────────────────────────────────────────────
  async function load() {
    loading = true;
    try {
      const r = await api.movements.list({ page, per_page: 15 });
      movements = r.data ?? [];
      total     = r.total ?? 0;
    } catch (e) { toast.error(e.message); }
    finally { loading = false; }
  }

  async function loadProducts() {
    try { const r = await api.products.list({ per_page: 100 }); products = r.data ?? []; }
    catch {}
  }

  onMount(() => { load(); loadProducts(); });

  // ── Modal ────────────────────────────────────────────────────────────────────
  function openModal()  { form = { product_id: '', movement_type: 'in', quantity: 1, reference: '', notes: '' }; showModal = true; }
  function closeModal() { showModal = false; }

  async function saveMovement() {
    if (!form.product_id) { toast.warning('Please select a product'); return; }
    try {
      const r = await api.movements.create({
        product_id:    form.product_id,
        movement_type: form.movement_type,
        quantity:      parseInt(form.quantity),
        reference:     form.reference || null,
        notes:         form.notes || null,
      });
      toast.success(`✅ Stock updated. New quantity: ${r.new_quantity}`);
      closeModal();
      load();
      window.dispatchEvent(new CustomEvent('stock-updated'));  // triggers dashboard refresh
    } catch (e) { toast.error(e.message); }
  }

  // ── Helpers ──────────────────────────────────────────────────────────────────
  const movBadgeClass = { in: 'badge-green', out: 'badge-red', adjustment: 'badge-amber', return: 'badge-cyan' };
  const movEmoji      = { in: '[IN]', out: '[OUT]', adjustment: '[ADJ]', return: '[RET]' };

  function fmtDate(dt) {
    return new Date(dt).toLocaleString('en-IN', {
      day: '2-digit', month: 'short', year: 'numeric',
      hour: '2-digit', minute: '2-digit'
    });
  }

  $: totalPages = Math.ceil(total / 15);
</script>

<div class="page">
  <div class="page-header">
    <div class="page-title-group">
      <h1 class="page-title">Stock Movements</h1>
      <p class="page-subtitle">{total} movements recorded — immutable audit log</p>
    </div>
    <button id="btn-new-movement" class="btn btn-primary" on:click={openModal}>+ Record Movement</button>
  </div>

  <!-- ── Legend ────────────────────────────────────────────────────────────── -->
  <div class="flex gap-3">
    {#each ['in','out','adjustment','return'] as t}
      <span class="badge {movBadgeClass[t]}">{movEmoji[t]} {t}</span>
    {/each}
  </div>

  <!-- ── Table ─────────────────────────────────────────────────────────────── -->
  <div class="card stagger-row" style="padding:0;overflow:hidden;flex:1;animation-delay: 50ms">
    {#if loading}
      <div style="display:flex;justify-content:center;padding:4rem"><div class="spinner" style="width:32px;height:32px;border-width:3px"></div></div>
    {:else}
      <div class="table-wrapper" style="border-radius:0;border:none;max-height:calc(100vh - 290px)">
        <table>
          <thead>
            <tr>
              <th>Date &amp; Time</th>
              <th>Product</th>
              <th>Type</th>
              <th>Quantity</th>
              <th>Reference</th>
              <th>Notes</th>
              <th>Performed By</th>
            </tr>
          </thead>
          <tbody>
            {#each movements as m, i (m.id)}
              <tr class="stagger-row" style="animation-delay: {100 + (i * 50)}ms">
                <td style="font-size:0.8125rem;color:var(--text-muted);white-space:nowrap">{fmtDate(m.created_at)}</td>
                <td>
                  <div style="font-weight:600">{m.product_name}</div>
                  <div style="font-size:0.75rem;font-family:var(--font-mono);color:var(--text-muted)">{m.product_sku}</div>
                </td>
                <td><span class="badge {movBadgeClass[m.movement_type]}">{movEmoji[m.movement_type]} {m.movement_type}</span></td>
                <td style="font-family:var(--font-mono);font-weight:700;font-size:1rem">{m.quantity}</td>
                <td style="color:var(--text-secondary);font-size:0.8125rem">{m.reference ?? '—'}</td>
                <td style="color:var(--text-muted);font-size:0.8125rem;max-width:200px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">{m.notes ?? '—'}</td>
                <td style="color:var(--text-secondary);font-size:0.8125rem">{m.performed_by_name}</td>
              </tr>
            {/each}
            {#if movements.length === 0}
              <tr><td colspan="7" style="text-align:center;color:var(--text-muted);padding:3rem">No movements recorded yet</td></tr>
            {/if}
          </tbody>
        </table>
      </div>

      {#if totalPages > 1}
        <div class="pagination" style="padding-bottom:var(--space-4)">
          <button class="page-btn" on:click={() => { page--; load(); }} disabled={page===1}>‹</button>
          {#each Array(Math.min(totalPages,7)) as _,i}
            {@const pg = i+1}
            <button class="page-btn" class:active={pg===page} on:click={() => { page=pg; load(); }}>{pg}</button>
          {/each}
          <button class="page-btn" on:click={() => { page++; load(); }} disabled={page>=totalPages}>›</button>
        </div>
      {/if}
    {/if}
  </div>
</div>

<!-- ── New Movement Modal ──────────────────────────────────────────────────── -->
{#if showModal}
  <div class="modal-backdrop" on:click|self={closeModal}>
    <div class="modal">
      <div class="modal-header">
        <h3>Record Stock Movement</h3>
        <button class="btn btn-ghost btn-icon" on:click={closeModal}>
          <X size={18} />
        </button>
      </div>
      <div class="modal-body">
        <div class="flex flex-col gap-4">
          <div class="form-group">
            <label class="label" for="mv-product">Product *</label>
            <select id="mv-product" class="select input" bind:value={form.product_id}>
              <option value="">— Select product —</option>
              {#each products as p}<option value={p.id}>[{p.sku}] {p.name} (stock: {p.quantity_in_stock})</option>{/each}
            </select>
          </div>
          <div class="grid grid-2 grid-gap-4">
            <div class="form-group">
              <label class="label" for="mv-type">Movement Type *</label>
              <select id="mv-type" class="select input" bind:value={form.movement_type}>
                <option value="in">Stock In (received)</option>
                <option value="out">Stock Out (dispatched)</option>
                <option value="adjustment">Adjustment (audit/damage)</option>
                <option value="return">Return (customer/supplier)</option>
              </select>
            </div>
            <div class="form-group">
              <label class="label" for="mv-qty">Quantity *</label>
              <input id="mv-qty" class="input" type="number" min="1" bind:value={form.quantity} />
            </div>
          </div>
          <div class="form-group">
            <label class="label" for="mv-ref">Reference (PO / Invoice / Order No.)</label>
            <input id="mv-ref" class="input" bind:value={form.reference} placeholder="e.g. PO-2024-001" />
          </div>
          <div class="form-group">
            <label class="label" for="mv-notes">Notes</label>
            <textarea id="mv-notes" class="textarea input" rows="2" bind:value={form.notes} placeholder="Optional notes about this movement…"></textarea>
          </div>
          <!-- Movement type hint -->
          <div class="hint-box">
            {#if form.movement_type === 'in'}
              <p><Package size={14} /> Stock will be <strong>added</strong> to the current quantity.</p>
            {:else if form.movement_type === 'out'}
              <p><Truck size={14} /> Stock will be <strong>deducted</strong>. Fails if insufficient.</p>
            {:else if form.movement_type === 'adjustment'}
              <p><Wrench size={14} /> Sets quantity to the value entered (audit correction).</p>
            {:else if form.movement_type === 'return'}
              <p><Undo2 size={14} /> Returned stock will be <strong>added</strong> back to inventory.</p>
            {/if}
          </div>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn btn-ghost" on:click={closeModal}>Cancel</button>
        <button id="btn-save-movement" class="btn btn-primary" on:click={saveMovement}>Record Movement</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .hint-box {
    background: rgba(124,58,237,0.08);
    border: 1px solid var(--border-subtle);
    border-radius: var(--radius-md);
    padding: var(--space-3) var(--space-4);
    font-size: 0.8125rem; color: var(--text-secondary);
  }
  .hint-box strong { color: var(--text-primary); }
</style>
