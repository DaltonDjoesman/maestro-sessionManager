import { pt } from "../i18n/pt";

interface RefreshIconButtonProps {
  onClick: () => void;
  disabled?: boolean;
  busy?: boolean;
}

export function RefreshIconButton({ onClick, disabled, busy }: RefreshIconButtonProps) {
  return (
    <button
      type="button"
      className={`btn-icon btn-refresh-icon${busy ? " btn-refresh-icon--busy" : ""}`}
      disabled={disabled}
      onClick={onClick}
      aria-label={pt.capture.refresh}
      title={pt.capture.refresh}
    >
      <svg width="18" height="18" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="2" aria-hidden>
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
        />
      </svg>
    </button>
  );
}
