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
    internal Text value;
    internal int has_value;

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

[StructLayout(LayoutKind.Sequential)]
internal struct Text_result
{
    internal int ok;
    internal Text_result_value value;

    internal static unsafe Text ToFacade(in Text_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static Text_result FromFacade(in Text self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new Text_result {ok = 1, value = new Text_result_value { ok = __item_from_facade } };
    }

    internal static Text_result FromFacadeError(in string err)
    {
        return new Text_result {ok = 0, value = new Text_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct Text_result_value
{
    [FieldOffset(0)]
    internal Text ok;

    [FieldOffset(0)]
    internal NativeString err;
}

[StructLayout(LayoutKind.Sequential)]
internal struct Text_optional_result
{
    internal int ok;
    internal Text_optional_result_value value;

    internal static unsafe Text? ToFacade(in Text_optional_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = Text_optional.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static Text_optional_result FromFacade(in Text? self)
    {
        var __item = self;
        var __item_from_facade = Text_optional.FromFacade(__item);
        return new Text_optional_result {ok = 1, value = new Text_optional_result_value { ok = __item_from_facade } };
    }

    internal static Text_optional_result FromFacadeError(in string err)
    {
        return new Text_optional_result {ok = 0, value = new Text_optional_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct Text_optional_result_value
{
    [FieldOffset(0)]
    internal Text_optional ok;

    [FieldOffset(0)]
    internal NativeString err;
}