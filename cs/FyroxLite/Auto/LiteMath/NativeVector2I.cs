// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite;

// fyrox_lite::lite_math::PodVector2I
[StructLayout(LayoutKind.Sequential)]
internal partial struct NativeVector2I
{
    private int _x;
    private int _y;
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeVector2I_optional
{
    private NativeVector2I value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Vector2I? ToFacade(in NativeVector2I_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = NativeVector2I.ToFacade(__item);
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static NativeVector2I_optional FromFacade(in Vector2I? value)
    {
        if (value == null)
        {
            return new NativeVector2I_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = NativeVector2I.FromFacade(__item);
        return new NativeVector2I_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct NativeVector2I_slice
{
    internal unsafe NativeVector2I* begin;
    internal int length;

    internal unsafe NativeVector2I_slice(NativeVector2I* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<Vector2I> ToFacade(in NativeVector2I_slice self)
    {
        var fetched = new List<Vector2I>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = NativeVector2I.ToFacade(__item);
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static NativeVector2I[]? _uploadBuffer;

    internal static NativeVector2I_slice FromFacade(in List<Vector2I> self)
    {
        _uploadBuffer ??= new NativeVector2I[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new NativeVector2I[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = NativeVector2I.FromFacade(__item);
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (NativeVector2I* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_fyrox_lite_lite_math_PodVector2I_slice(new NativeVector2I_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial NativeVector2I_slice fyrox_lite_upload_fyrox_lite_lite_math_PodVector2I_slice(NativeVector2I_slice managed);
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeVector2I_result
{
    internal int ok;
    internal NativeVector2I_result_value value;

    internal static unsafe Vector2I ToFacade(in NativeVector2I_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = NativeVector2I.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static NativeVector2I_result FromFacade(in Vector2I self)
    {
        var __item = self;
        var __item_from_facade = NativeVector2I.FromFacade(__item);
        return new NativeVector2I_result {ok = 1, value = new NativeVector2I_result_value { ok = __item_from_facade } };
    }

    internal static NativeVector2I_result FromFacadeError(in string err)
    {
        return new NativeVector2I_result {ok = 0, value = new NativeVector2I_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct NativeVector2I_result_value
{
    [FieldOffset(0)]
    internal NativeVector2I ok;

    [FieldOffset(0)]
    internal NativeString err;
}