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

// fyrox_lite::lite_ui::LiteText
public partial struct Text : IEquatable<Text>
{
    private readonly NativeHandle handle;

    internal Text(NativeHandle handle)
    {
        this.handle = handle;
    }
    public string TextAsync
    {
        set
        {
            unsafe {
                var _value = NativeString.FromFacade(value);
                fyrox_lite_lite_ui_LiteText_set_text_async(this, _value);
            }
        }
    }

    public static Text New(TextBuilder state)
    {
        unsafe {
            var _state = state;
            var __ret = fyrox_lite_lite_ui_LiteText_new(&_state);
            return __ret;
        }
    }

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_ui_LiteText_set_text_async(Text self, NativeString text);

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Text fyrox_lite_lite_ui_LiteText_new(TextBuilder* state);

    public bool Equals(Text other)
    {
        return handle.Equals(other.handle);
    }

    public override bool Equals(object? obj)
    {
        return obj is Text other && Equals(other);
    }

    public override int GetHashCode()
    {
        return handle.GetHashCode();
    }

    public static bool operator ==(Text left, Text right)
    {
        return left.Equals(right);
    }

    public static bool operator !=(Text left, Text right)
    {
        return !left.Equals(right);
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct Text_optional
{
    private Text value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Text? ToFacade(in Text_optional value)
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
    public static Text_optional FromFacade(in Text? value)
    {
        if (value == null)
        {
            return new Text_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new Text_optional { value = __item_from_facade, has_value = 1 };
    }
}