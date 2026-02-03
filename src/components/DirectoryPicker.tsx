import { dialogApi } from '../api';

interface DirectoryPickerProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  disabled?: boolean;
}

export const DirectoryPicker: React.FC<DirectoryPickerProps> = ({
  value,
  onChange,
  placeholder = '选择工作目录',
  disabled = false,
}) => {
  const handleBrowse = async () => {
    try {
      const selected = await dialogApi.selectDirectory();
      if (selected) {
        onChange(selected);
      }
    } catch (error) {
      console.error('选择目录失败:', error);
    }
  };

  return (
    <div className="flex items-center gap-2">
      <input
        type="text"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        disabled={disabled}
        className="flex-1 px-3 py-2 bg-[#343638] border border-[#565B5E] rounded text-[12px] disabled:opacity-50"
      />
      <button
        type="button"
        onClick={handleBrowse}
        disabled={disabled}
        className="px-4 py-2 text-[12px] bg-[#565B5E] hover:bg-[#7A8488] text-white rounded disabled:opacity-50"
      >
        选择或拖入
      </button>
    </div>
  );
};
