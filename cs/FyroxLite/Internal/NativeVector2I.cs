namespace FyroxLite;

internal partial struct NativeVector2I
{
    internal NativeVector2I(int x, int y)
    {
        _x = x;
        _y = y;
    }
    internal static Vector2I ToFacade(NativeVector2I self)
    {
        return new Vector2I(self._x, self._y);
    }

    internal static NativeVector2I FromFacade(Vector2I self)
    {
        return new NativeVector2I(self.X, self.Y);
    }
}