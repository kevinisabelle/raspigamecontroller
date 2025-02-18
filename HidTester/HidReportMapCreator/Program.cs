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

Console.WriteLine(reportPayload.GetPayloadLine(1));
Console.WriteLine(reportPayload.GetPayloadLine(2));



Console.WriteLine();
AnsiConsole.MarkupLine("[blue]------------[/]");
AnsiConsole.MarkupLine("[blue]Python class[/]");
AnsiConsole.MarkupLine("[blue]------------[/]");
Console.WriteLine();

Console.WriteLine(reportPayload.GeneratePythonClassCode());
Console.WriteLine();
Console.WriteLine(reportMap.GeneratePythonGetReportMapFunction(reportMap));

Console.ReadLine();