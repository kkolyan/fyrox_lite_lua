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

// fyrox_lite::lite_physics::LiteIntersection
[StructLayout(LayoutKind.Sequential)]
public partial struct Intersection
{
    public Node Collider {
        get => _collider;
        set => _collider = value;
    }
    public Vector3 Normal {
        get => NativeVector3.ToFacade(_normal);
        set => _normal = NativeVector3.FromFacade(value);
    }
    public Vector3 Position {
        get => NativeVector3.ToFacade(_position);
        set => _position = NativeVector3.FromFacade(value);
    }
    public FeatureId Feature {
        get => _feature;
        set => _feature = value;
    }
    public float Toi {
        get => _toi;
        set => _toi = value;
    }
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private Node _collider;
    private NativeVector3 _normal;
    private NativeVector3 _position;
    private FeatureId _feature;
    private float _toi;
}

[StructLayout(LayoutKind.Sequential)]
internal struct Intersection_optional
{
    internal Intersection value;
    internal int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Intersection? ToFacade(in Intersection_optional value)
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
    public static Intersection_optional FromFacade(in Intersection? value)
    {
        if (value == null)
        {
            return new Intersection_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new Intersection_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct Intersection_slice
{
    internal unsafe Intersection* begin;
    internal int length;

    internal unsafe Intersection_slice(Intersection* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<Intersection> ToFacade(in Intersection_slice self)
    {
        var fetched = new List<Intersection>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static Intersection[]? _uploadBuffer;

    internal static Intersection_slice FromFacade(in List<Intersection> self)
    {
        _uploadBuffer ??= new Intersection[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new Intersection[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (Intersection* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_fyrox_lite_lite_physics_LiteIntersection_slice(new Intersection_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("libfyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial Intersection_slice fyrox_lite_upload_fyrox_lite_lite_physics_LiteIntersection_slice(Intersection_slice managed);
}

[StructLayout(LayoutKind.Sequential)]
internal struct Intersection_result
{
    internal int ok;
    internal Intersection_result_value value;

    internal static unsafe Intersection ToFacade(in Intersection_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static Intersection_result FromFacade(in Intersection self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new Intersection_result {ok = 1, value = new Intersection_result_value { ok = __item_from_facade } };
    }

    internal static Intersection_result FromFacadeError(in string err)
    {
        return new Intersection_result {ok = 0, value = new Intersection_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct Intersection_result_value
{
    [FieldOffset(0)]
    internal Intersection ok;

    [FieldOffset(0)]
    internal NativeString err;
}