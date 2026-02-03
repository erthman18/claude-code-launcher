interface OnboardingTriggerProps {
  onClick: () => void;
}

export const OnboardingTrigger: React.FC<OnboardingTriggerProps> = ({ onClick }) => {
  return (
    <button
      onClick={onClick}
      className="fixed bottom-4 right-4 w-8 h-8 rounded-full bg-[#3a3a3a]
                 hover:bg-[#4a4a4a] text-[#999999] hover:text-white
                 flex items-center justify-center transition-colors z-40"
      title="查看新手引导"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      >
        <circle cx="12" cy="12" r="10" />
        <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" />
        <line x1="12" y1="17" x2="12.01" y2="17" />
      </svg>
    </button>
  );
};
