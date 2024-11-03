using System.Numerics;

namespace FyroxLite;

internal partial struct NativeVector2
{
    internal NativeVector2(float x, float y)
    {
        _x = x;
        _y = y;
    }
    internal static Vector2 ToFacade(NativeVector2 self)
    {
        return new Vector2(self._x, self._y);
    }

    internal static NativeVector2 FromFacade(Vector2 self)
    {
        return new NativeVector2(self.X, self.Y);
    }
}