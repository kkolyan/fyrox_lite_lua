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
namespace FyroxLite.LiteUi;

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
    private GradientPoint value;
    private int has_value;

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

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial GradientPoint_slice fyrox_lite_upload_fyrox_lite_lite_ui_GradientPoint_slice(GradientPoint_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct GradientPoint_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private GradientPoint value;

    [FieldOffset(sizeof(int))]
    private NativeString err;

    internal static unsafe GradientPoint ToFacade(in GradientPoint_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.err));
    }

    internal static GradientPoint_result FromFacade(in GradientPoint self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new GradientPoint_result {ok = 1, value = __item_from_facade};
    }
}