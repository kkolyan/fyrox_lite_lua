
using System.Reflection;
using System.Text.RegularExpressions;
using FyroxLite.___Internal;

var assemblies = new List<Assembly>();

foreach (var file in Directory.GetFiles("."))
{
    if (!file.EndsWith(".csproj"))
    {
        continue;
    }
    Console.WriteLine($"found project: {file}");
    var name = Regex.Replace(file, @"\./(.*)\.csproj", "$1");
    var assembly = Assembly.LoadFrom($"bin/Debug/net8.0/{name}.dll");
    
    Console.WriteLine($"loaded assembly: {assembly}");
    assemblies.Add(assembly);
}

if (assemblies.Count == 0)
{
    Console.WriteLine($"No script assemblies found in directory {Directory.GetCurrentDirectory()}");
    return;
}
    
___FyroxExecutor.__Run(assemblies);
