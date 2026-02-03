import React from 'react';

interface ConfirmDialogProps {
  isOpen: boolean;
  title: string;
  message: string;
  confirmLabel?: string;
  cancelLabel?: string;
  onConfirm: () => void;
  onCancel: () => void;
  variant?: 'danger' | 'default';
}

export const ConfirmDialog: React.FC<ConfirmDialogProps> = ({
  isOpen,
  title,
  message,
  confirmLabel = '确认',
  cancelLabel = '取消',
  onConfirm,
  onCancel,
  variant = 'default',
}) => {
  if (!isOpen) return null;

  const confirmButtonClass =
    variant === 'danger'
      ? 'bg-red-600 hover:bg-red-700'
      : 'bg-[#3b82f6] hover:bg-[#2563eb]';

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* 背景遮罩 */}
      <div
        className="absolute inset-0 bg-black/60"
        onClick={onCancel}
      />

      {/* 对话框 */}
      <div className="relative bg-[#2a2a2a] border border-[#565B5E] rounded-lg shadow-xl max-w-md w-full mx-4 p-6">
        <h3 className="text-[16px] font-bold text-[#DCE4EE] mb-3">{title}</h3>
        <p className="text-[14px] text-[#999999] mb-6 whitespace-pre-line">{message}</p>

        <div className="flex justify-end gap-3">
          <button
            type="button"
            onClick={onCancel}
            className="px-4 py-2 text-[12px] bg-[#565B5E] hover:bg-[#7A8488] text-white rounded"
          >
            {cancelLabel}
          </button>
          <button
            type="button"
            onClick={onConfirm}
            className={`px-4 py-2 text-[12px] ${confirmButtonClass} text-white rounded`}
          >
            {confirmLabel}
          </button>
        </div>
      </div>
    </div>
  );
};
