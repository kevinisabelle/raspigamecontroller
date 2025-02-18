using HidReportMapCreator.Devices;
using HidReportMapCreator.Translation;
using Spectre.Console;

AnsiConsole.MarkupLine("[bold yellow]Hid Report Map Creator[/]");

var device = new KiGPSimple().Create();

var reportMap = device.ToReportMap();
var reportPayload = device.ToReportPayload();

Console.WriteLine();
AnsiConsole.MarkupLine("[blue]---------------------[/]");
AnsiConsole.MarkupLine("[blue]Report Map Descriptor[/]");
AnsiConsole.MarkupLine("[blue]---------------------[/]");
Console.WriteLine();

foreach (var instruction in reportMap.Instructions)
{
    // Write hex formatted data bytes
    AnsiConsole.Markup("[aqua]" + string.Join("[/], [aqua]", instruction.Data.Select(b => "0x" + b.ToString("X2"))) + "[/]");
    AnsiConsole.Markup("\t[blue]# " + instruction.Comment + "[/]");
    Console.WriteLine();
}

Console.WriteLine();
AnsiConsole.MarkupLine("[blue]----------------------[/]");
AnsiConsole.MarkupLine("[blue]Report Payload Example[/]");
AnsiConsole.MarkupLine("[blue]----------------------[/]");
Console.WriteLine();

foreach (var field in reportPayload.Fields)
{
    if (field.Index == -1)
    {
        AnsiConsole.MarkupLine($"[red]Padding:[/] ([yellow]{field.Input.GetReportPaddingSize()} bits[/])");
    } 
    else 
    {
        AnsiConsole.MarkupLine($"{field.Input.Name} [blue]#{field.Index+1}[/] ([yellow]{field.Input.GetValueBitSize()} bits[/]) {field.Comment} {(field.Padding > 0 ? $"[red]Padding:[/] ([yellow]{field.Padding} bits[/])" : "")}");
    }
}

Console.WriteLine();
AnsiConsole.MarkupLine("[blue]----------------------------------[/]");
AnsiConsole.MarkupLine("[blue]Report Payload Example (Formatted)[/]");
AnsiConsole.MarkupLine("[blue]----------------------------------[/]");
Console.WriteLine();
string result = "";
string fieldStr = "";

foreach (var field in reportPayload.Fields)
{
    fieldStr = field.Input.Name.Substring(0, 1);

    if (field.Index == -1)
    {
        fieldStr = "_";
    }
    
    var numberOfOccurences = field.BitSize;
    
    // Repeat the full field string the number of times it occurs
    result += new string(Enumerable.Range(0, numberOfOccurences).SelectMany(i => fieldStr).ToArray());
    
    if (field.Padding > 0)
    {
        // Print the padding character the number of times it occurs
        result += new string(Enumerable.Range(0, field.Padding).SelectMany(i => "_".ToCharArray()).ToArray());
    }
}

result = string.Join(" ", Enumerable.Range(0, result.Length / 8).Select(i => result.Substring(i * 8, 8)));

Console.WriteLine(result);
result = "";
foreach (var field in reportPayload.Fields)
{
    fieldStr = (field.Index+1).ToString();

    if (field.Index == -1)
    {
        fieldStr = "_";
    }
    
    var numberOfOccurences = field.BitSize;
    
    // Repeat the full field string the number of times it occurs
    result += new string(Enumerable.Range(0, numberOfOccurences).SelectMany(i => fieldStr).ToArray());
    
    if (field.Padding > 0)
    {
        // Print the padding character the number of times it occurs
        result += new string(Enumerable.Range(0, field.Padding).SelectMany(i => "_".ToCharArray()).ToArray());
    }
}

result = string.Join(" ", Enumerable.Range(0, result.Length / 8).Select(i => result.Substring(i * 8, 8)));

Console.WriteLine(result);

Console.ReadLine();