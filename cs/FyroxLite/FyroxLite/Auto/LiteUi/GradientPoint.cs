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

// fyrox_lite::lite_ui::GradientPoint
[StructLayout(LayoutKind.Sequential)]
public partial struct GradientPoint
{
    public float Stop {
        get => _stop;
        set => _stop = value;
    }
    public Color Color {
        get => NativeColor.ToFacade(_color);
        set => _color = NativeColor.FromFacade(value);
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private float _stop;
    private NativeColor _color;
}

[StructLayout(LayoutKind.Sequential)]
internal struct GradientPoint_optional
{
    internal GradientPoint value;
    internal int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static GradientPoint? ToFacade(in GradientPoint_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static GradientPoint_optional FromFacade(in GradientPoint? value)
    {
        if (value == null)
        {
            return new GradientPoint_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new GradientPoint_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct GradientPoint_slice
{
    internal unsafe GradientPoint* begin;
    internal int length;

    internal unsafe GradientPoint_slice(GradientPoint* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<GradientPoint> ToFacade(in GradientPoint_slice self)
    {
        var fetched = new List<GradientPoint>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static GradientPoint[]? _uploadBuffer;

    internal static GradientPoint_slice FromFacade(in List<GradientPoint> self)
    {
        _uploadBuffer ??= new GradientPoint[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new GradientPoint[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (GradientPoint* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_fyrox_lite_lite_ui_GradientPoint_slice(new GradientPoint_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial GradientPoint_slice fyrox_lite_upload_fyrox_lite_lite_ui_GradientPoint_slice(GradientPoint_slice managed);
}

[StructLayout(LayoutKind.Sequential)]
internal struct GradientPoint_result
{
    internal int ok;
    internal GradientPoint_result_value value;

    internal static unsafe GradientPoint ToFacade(in GradientPoint_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static GradientPoint_result FromFacade(in GradientPoint self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new GradientPoint_result {ok = 1, value = new GradientPoint_result_value { ok = __item_from_facade } };
    }

    internal static GradientPoint_result FromFacadeError(in string err)
    {
        return new GradientPoint_result {ok = 0, value = new GradientPoint_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct GradientPoint_result_value
{
    [FieldOffset(0)]
    internal GradientPoint ok;

    [FieldOffset(0)]
    internal NativeString err;
}