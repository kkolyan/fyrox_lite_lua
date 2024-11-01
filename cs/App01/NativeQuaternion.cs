using System.Numerics;

namespace FyroxLite.LiteMath;

internal partial struct NativeQuaternion
{
    
    internal NativeQuaternion(float i, float j, float k, float w)
    {
        _i = i;
        _j = j;
        _k = k;
        _w = w;
    }
    internal static Quaternion ToFacade(NativeQuaternion self)
    {
        return new Quaternion(self._i, self._j, self._k, self._w);
    }

    internal static NativeQuaternion FromFacade(Quaternion self)
    {
        return new NativeQuaternion(self.X, self.Y, self.Z, self.W);
    }
}