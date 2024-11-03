// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite;
using System.Numerics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite;

// fyrox_lite::lite_math::PodQuaternion
[StructLayout(LayoutKind.Sequential)]
internal partial struct NativeQuaternion
{
    private float _i;
    private float _j;
    private float _k;
    private float _w;
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeQuaternion_optional
{
    private NativeQuaternion value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Quaternion? ToFacade(in NativeQuaternion_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = NativeQuaternion.ToFacade(__item);
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static NativeQuaternion_optional FromFacade(in Quaternion? value)
    {
        if (value == null)
        {
            return new NativeQuaternion_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = NativeQuaternion.FromFacade(__item);
        return new NativeQuaternion_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct NativeQuaternion_slice
{
    internal unsafe NativeQuaternion* begin;
    internal int length;

    internal unsafe NativeQuaternion_slice(NativeQuaternion* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<Quaternion> ToFacade(in NativeQuaternion_slice self)
    {
        var fetched = new List<Quaternion>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = NativeQuaternion.ToFacade(__item);
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static NativeQuaternion[]? _uploadBuffer;

    internal static NativeQuaternion_slice FromFacade(in List<Quaternion> self)
    {
        _uploadBuffer ??= new NativeQuaternion[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new NativeQuaternion[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = NativeQuaternion.FromFacade(__item);
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (NativeQuaternion* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_fyrox_lite_lite_math_PodQuaternion_slice(new NativeQuaternion_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial NativeQuaternion_slice fyrox_lite_upload_fyrox_lite_lite_math_PodQuaternion_slice(NativeQuaternion_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct NativeQuaternion_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private NativeQuaternion value;

    [FieldOffset(sizeof(int))]
    private NativeString err;

    internal static unsafe Quaternion ToFacade(in NativeQuaternion_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = NativeQuaternion.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.err));
    }

    internal static NativeQuaternion_result FromFacade(in Quaternion self)
    {
        var __item = self;
        var __item_from_facade = NativeQuaternion.FromFacade(__item);
        return new NativeQuaternion_result {ok = 1, value = __item_from_facade};
    }

    internal static NativeQuaternion_result FromFacadeError(in string err)
    {
        return new NativeQuaternion_result {ok = 0, err = NativeString.FromFacade(err)};
    }
}