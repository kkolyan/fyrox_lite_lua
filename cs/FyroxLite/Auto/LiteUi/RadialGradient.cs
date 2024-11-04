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

// fyrox_lite::lite_ui::RadialGradient
[StructLayout(LayoutKind.Sequential)]
public partial struct RadialGradient
{
    public Vector2 Center {
        get => NativeVector2.ToFacade(_center);
        set => _center = NativeVector2.FromFacade(value);
    }
    public List<GradientPoint> Stops {
        get => GradientPoint_slice.ToFacade(_stops);
        set => _stops = GradientPoint_slice.FromFacade(value);
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private NativeVector2 _center;
    private GradientPoint_slice _stops;
}

[StructLayout(LayoutKind.Sequential)]
internal struct RadialGradient_optional
{
    private RadialGradient value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static RadialGradient? ToFacade(in RadialGradient_optional value)
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
    public static RadialGradient_optional FromFacade(in RadialGradient? value)
    {
        if (value == null)
        {
            return new RadialGradient_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new RadialGradient_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct RadialGradient_slice
{
    internal unsafe RadialGradient* begin;
    internal int length;

    internal unsafe RadialGradient_slice(RadialGradient* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<RadialGradient> ToFacade(in RadialGradient_slice self)
    {
        var fetched = new List<RadialGradient>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static RadialGradient[]? _uploadBuffer;

    internal static RadialGradient_slice FromFacade(in List<RadialGradient> self)
    {
        _uploadBuffer ??= new RadialGradient[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new RadialGradient[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (RadialGradient* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_fyrox_lite_lite_ui_RadialGradient_slice(new RadialGradient_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial RadialGradient_slice fyrox_lite_upload_fyrox_lite_lite_ui_RadialGradient_slice(RadialGradient_slice managed);
}

[StructLayout(LayoutKind.Sequential)]
internal struct RadialGradient_result
{
    internal int ok;
    internal RadialGradient_result_value value;

    internal static unsafe RadialGradient ToFacade(in RadialGradient_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static RadialGradient_result FromFacade(in RadialGradient self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new RadialGradient_result {ok = 1, value = new RadialGradient_result_value { ok = __item_from_facade } };
    }

    internal static RadialGradient_result FromFacadeError(in string err)
    {
        return new RadialGradient_result {ok = 0, value = new RadialGradient_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct RadialGradient_result_value
{
    [FieldOffset(0)]
    internal RadialGradient ok;

    [FieldOffset(0)]
    internal NativeString err;
}