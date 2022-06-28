using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace StarBooks.Infrastructure.Migrations
{
    public partial class EnableMoreNullableFields : Migration
    {
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.AlterColumn<string>(
                name: "VolumeInfo_Language",
                table: "books",
                type: "nvarchar(max)",
                nullable: true,
                oldClrType: typeof(string),
                oldType: "nvarchar(max)");

            migrationBuilder.AlterColumn<string>(
                name: "VolumeInfo_ImageLinks_ThumbnailUrl",
                table: "books",
                type: "nvarchar(max)",
                nullable: true,
                oldClrType: typeof(string),
                oldType: "nvarchar(max)");

            migrationBuilder.AlterColumn<string>(
                name: "VolumeInfo_Description",
                table: "books",
                type: "nvarchar(max)",
                nullable: true,
                oldClrType: typeof(string),
                oldType: "nvarchar(max)");

            migrationBuilder.AlterColumn<string>(
                name: "AccessInfo_Country",
                table: "books",
                type: "nvarchar(max)",
                nullable: true,
                oldClrType: typeof(string),
                oldType: "nvarchar(max)");

            migrationBuilder.AddColumn<decimal>(
                name: "SaleInfo_ListPrice_Amount",
                table: "books",
                type: "decimal(18,2)",
                nullable: true);

            migrationBuilder.AddColumn<string>(
                name: "SaleInfo_ListPrice_CurrencyCode",
                table: "books",
                type: "nvarchar(max)",
                nullable: true);

            migrationBuilder.AddColumn<decimal>(
                name: "SaleInfo_RetailPrice_Amount",
                table: "books",
                type: "decimal(18,2)",
                nullable: true);

            migrationBuilder.AddColumn<string>(
                name: "SaleInfo_RetailPrice_CurrencyCode",
                table: "books",
                type: "nvarchar(max)",
                nullable: true);
        }

        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropColumn(
                name: "SaleInfo_ListPrice_Amount",
                table: "books");

            migrationBuilder.DropColumn(
                name: "SaleInfo_ListPrice_CurrencyCode",
                table: "books");

            migrationBuilder.DropColumn(
                name: "SaleInfo_RetailPrice_Amount",
                table: "books");

            migrationBuilder.DropColumn(
                name: "SaleInfo_RetailPrice_CurrencyCode",
                table: "books");

            migrationBuilder.AlterColumn<string>(
                name: "VolumeInfo_Language",
                table: "books",
                type: "nvarchar(max)",
                nullable: false,
                defaultValue: "",
                oldClrType: typeof(string),
                oldType: "nvarchar(max)",
                oldNullable: true);

            migrationBuilder.AlterColumn<string>(
                name: "VolumeInfo_ImageLinks_ThumbnailUrl",
                table: "books",
                type: "nvarchar(max)",
                nullable: false,
                defaultValue: "",
                oldClrType: typeof(string),
                oldType: "nvarchar(max)",
                oldNullable: true);

            migrationBuilder.AlterColumn<string>(
                name: "VolumeInfo_Description",
                table: "books",
                type: "nvarchar(max)",
                nullable: false,
                defaultValue: "",
                oldClrType: typeof(string),
                oldType: "nvarchar(max)",
                oldNullable: true);

            migrationBuilder.AlterColumn<string>(
                name: "AccessInfo_Country",
                table: "books",
                type: "nvarchar(max)",
                nullable: false,
                defaultValue: "",
                oldClrType: typeof(string),
                oldType: "nvarchar(max)",
                oldNullable: true);
        }
    }
}
