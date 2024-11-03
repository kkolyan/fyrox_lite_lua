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

// fyrox_lite::lite_ui::TextBuilder
[StructLayout(LayoutKind.Sequential)]
public partial struct TextBuilder
{
    public Brush? Foreground {
        get => Brush_optional.ToFacade(_foreground);
        set => _foreground = Brush_optional.FromFacade(value);
    }
    public float? FontSize {
        get => float_optional.ToFacade(_font_size);
        set => _font_size = float_optional.FromFacade(value);
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private Brush_optional _foreground;
    private float_optional _font_size;
}

[StructLayout(LayoutKind.Sequential)]
internal struct TextBuilder_optional
{
    private TextBuilder value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static TextBuilder? ToFacade(in TextBuilder_optional value)
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
    public static TextBuilder_optional FromFacade(in TextBuilder? value)
    {
        if (value == null)
        {
            return new TextBuilder_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new TextBuilder_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct TextBuilder_slice
{
    internal unsafe TextBuilder* begin;
    internal int length;

    internal unsafe TextBuilder_slice(TextBuilder* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<TextBuilder> ToFacade(in TextBuilder_slice self)
    {
        var fetched = new List<TextBuilder>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static TextBuilder[]? _uploadBuffer;

    internal static TextBuilder_slice FromFacade(in List<TextBuilder> self)
    {
        _uploadBuffer ??= new TextBuilder[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new TextBuilder[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (TextBuilder* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_fyrox_lite_lite_ui_TextBuilder_slice(new TextBuilder_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial TextBuilder_slice fyrox_lite_upload_fyrox_lite_lite_ui_TextBuilder_slice(TextBuilder_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct TextBuilder_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private TextBuilder value;

    [FieldOffset(sizeof(int))]
    private NativeString err;

    internal static unsafe TextBuilder ToFacade(in TextBuilder_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.err));
    }

    internal static TextBuilder_result FromFacade(in TextBuilder self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new TextBuilder_result {ok = 1, value = __item_from_facade};
    }

    internal static TextBuilder_result FromFacadeError(in string err)
    {
        return new TextBuilder_result {ok = 0, err = NativeString.FromFacade(err)};
    }
}