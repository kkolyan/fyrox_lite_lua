using System.Numerics;

namespace FyroxLite.LiteMath;

internal partial struct NativeVector3
{
    internal NativeVector3(float x, float y, float z)
    {
        _x = x;
        _y = y;
        _z = z;
    }
    internal static Vector3 ToFacade(NativeVector3 self)
    {
        return new Vector3(self._x, self._y, self._z);
    }

    internal static NativeVector3 FromFacade(Vector3 self)
    {
        return new NativeVector3(self.X, self.Y, self.Z);
    }
}