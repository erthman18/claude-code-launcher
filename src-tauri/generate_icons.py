"""
高质量图标生成脚本
使用 LANCZOS 重采样算法生成清晰的多尺寸图标
"""
from PIL import Image
import os

# 切换到脚本所在目录
os.chdir(os.path.dirname(os.path.abspath(__file__)))

# 配置
SOURCE_ICON = "icons/icon.png"  # 512x512 源图标
ICO_SIZES = [16, 24, 32, 48, 64, 128, 256, 512]  # 包含 512 用于高 DPI 显示
PNG_SIZES = {
    "icons/32x32.png": 32,
    "icons/128x128.png": 128,
    "icons/128x128@2x.png": 256,
}

def main():
    # 加载源图标
    print(f"加载源图标: {SOURCE_ICON}")
    img = Image.open(SOURCE_ICON)
    print(f"源图标尺寸: {img.size}")

    # 确保是 RGBA 模式
    if img.mode != "RGBA":
        img = img.convert("RGBA")

    # 生成各尺寸 PNG（使用 LANCZOS 高质量重采样）
    print("\n生成 PNG 图标:")
    for path, size in PNG_SIZES.items():
        resized = img.resize((size, size), Image.LANCZOS)
        resized.save(path, "PNG", optimize=True)
        file_size = os.path.getsize(path)
        print(f"  {path}: {size}x{size}, {file_size} bytes")

    # 生成高质量 ICO
    print("\n生成 ICO 图标:")
    ico_images = []
    for size in ICO_SIZES:
        resized = img.resize((size, size), Image.LANCZOS)
        ico_images.append(resized)
        print(f"  添加 {size}x{size}")

    # 保存 ICO 文件 - 使用正确的方式
    ico_path = "icons/icon.ico"

    # Pillow 需要从最大的图像开始保存，并指定所有尺寸
    # 反转列表让最大的在前面
    ico_images_reversed = ico_images[::-1]
    sizes_reversed = ICO_SIZES[::-1]

    ico_images_reversed[0].save(
        ico_path,
        format="ICO",
        append_images=ico_images_reversed[1:],
        sizes=[(s, s) for s in sizes_reversed]
    )
    ico_size = os.path.getsize(ico_path)
    print(f"\nICO 文件已保存: {ico_path} ({ico_size} bytes)")
    print(f"包含尺寸: {ICO_SIZES}")

    print("\n完成!")

if __name__ == "__main__":
    main()
