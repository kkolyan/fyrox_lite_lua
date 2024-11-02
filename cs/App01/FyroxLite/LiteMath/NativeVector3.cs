// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite.LiteInput;
using FyroxLite.LiteLog;
using FyroxLite.LiteMath;
using FyroxLite.LiteNode;
using FyroxLite.LitePhysics;
using FyroxLite.LitePlugin;
using FyroxLite.LitePrefab;
using FyroxLite.LiteScene;
using FyroxLite.LiteUi;
using FyroxLite.LiteWindow;
using System.Numerics;
using System.Drawing;
using FyroxLite.LiteBase;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite.LiteMath;

// fyrox_lite::lite_math::PodVector3
[StructLayout(LayoutKind.Sequential)]
internal partial struct NativeVector3
{
    private float _x;
    private float _y;
    private float _z;
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeVector3_optional
{
    private NativeVector3 value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Vector3? ToFacade(in NativeVector3_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = NativeVector3.ToFacade(__item);
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static NativeVector3_optional FromFacade(in Vector3? value)
    {
        if (value == null)
        {
            return new NativeVector3_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = NativeVector3.FromFacade(__item);
        return new NativeVector3_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct NativeVector3_slice
{
    internal unsafe NativeVector3* begin;
    internal int length;

    internal unsafe NativeVector3_slice(NativeVector3* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<Vector3> ToFacade(in NativeVector3_slice self)
    {
        var fetched = new List<Vector3>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = NativeVector3.ToFacade(__item);
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static NativeVector3[]? _uploadBuffer;

    internal static NativeVector3_slice FromFacade(in List<Vector3> self)
    {
        _uploadBuffer ??= new NativeVector3[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new NativeVector3[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = NativeVector3.FromFacade(__item);
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (NativeVector3* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_fyrox_lite_lite_math_PodVector3_slice(new NativeVector3_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeVector3_slice fyrox_lite_upload_fyrox_lite_lite_math_PodVector3_slice(NativeVector3_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct NativeVector3_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private NativeVector3 value;

    [FieldOffset(sizeof(int))]
    private NativeString err;

    internal static unsafe Vector3 ToFacade(in NativeVector3_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = NativeVector3.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.err));
    }

    internal static NativeVector3_result FromFacade(in Vector3 self)
    {
        var __item = self;
        var __item_from_facade = NativeVector3.FromFacade(__item);
        return new NativeVector3_result {ok = 1, value = __item_from_facade};
    }
}