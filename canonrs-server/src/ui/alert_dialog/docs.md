# AlertDialog

## When to use
Use for destructive or irreversible actions that require explicit user confirmation (delete, logout, cancel subscription).

## Composition
```
<AlertDialog>
  <AlertDialogTrigger />
  <AlertDialogOverlay />
  <AlertDialogContent>
    <AlertDialogTitle />
    <AlertDialogDescription />
    <AlertDialogClose />
  </AlertDialogContent>
</AlertDialog>
```

## Accessibility
- Uses `role="alertdialog"`
- Focus trapped inside dialog when open
- `aria-labelledby` and `aria-describedby` generated automatically by primitive
- Escape key closes dialog

## Behavior
- Open/close managed by behavior engine via `data-rs-alert-dialog`
- Overlay click closes dialog
- Close button uses `data-rs-close`

## Edge cases
- Always provide both Title and Description for screen readers
- Do not use for non-destructive confirmations — use Dialog instead
