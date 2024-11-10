using System.Drawing;

// ReSharper disable InconsistentNaming

namespace FyroxLite;

internal partial struct NativeColor
{
    private const int ARGBAlphaShift = 24;
    private const int ARGBRedShift = 16;
    private const int ARGBGreenShift = 8;
    private const int ARGBBlueShift = 0;

    internal NativeColor(byte r, byte g, byte b, byte a)
    {
        _r = r;
        _g = g;
        _b = b;
        _a = a;
    }

    public static Color ToFacade(NativeColor value)
    {
        return Color.FromArgb(
            value._a << ARGBAlphaShift |
            value._r << ARGBRedShift |
            value._g << ARGBGreenShift |
            value._b << ARGBBlueShift);
    }

    public static NativeColor FromFacade(Color value)
    {
        var argb = value.ToArgb();
        return new NativeColor(
            unchecked((byte)(argb >> ARGBRedShift)),
            unchecked((byte)(argb >> ARGBGreenShift)),
            unchecked((byte)(argb >> ARGBBlueShift)),
            unchecked((byte)(argb >> ARGBAlphaShift))
        );
    }
}