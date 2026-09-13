#!/usr/bin/env python3
"""Update explicit H1 year markers, preserving all other HTML source bytes."""

import argparse
from datetime import datetime, timedelta, timezone
from html.parser import HTMLParser
import os
from pathlib import Path
import re
import sys
from zoneinfo import ZoneInfo, ZoneInfoNotFoundError


ROOT = Path(__file__).resolve().parent.parent
MARKER = '<span data-guide-year="current">'
SKIP_DIRS = {".git", ".venv", "venv", "node_modules", "__pycache__", "target"}


def current_kst_year():
    try:
        seoul = ZoneInfo("Asia/Seoul")
    except ZoneInfoNotFoundError:
        # Windows may lack IANA data. Modern Korea uses UTC+09:00 without DST.
        seoul = timezone(timedelta(hours=9), "Asia/Seoul")
    return datetime.now(seoul).year


class GuideYearParser(HTMLParser):
    """Locate real elements, ignoring marker-like text in comments/scripts."""

    def __init__(self, source):
        super().__init__(convert_charrefs=False)
        self.source = source
        self.line_offsets = [0] + [m.end() for m in re.finditer("\n", source)]
        self.h1_count = 0
        self.in_h1 = False
        self.pending = None
        self.years = []

    def source_offset(self):
        line, column = self.getpos()
        return self.line_offsets[line - 1] + column

    def handle_starttag(self, tag, attrs):
        if self.pending is not None:
            raise ValueError("year marker must contain only four ASCII digits")
        if tag == "h1":
            self.h1_count += 1
            self.in_h1 = True
        if ("data-guide-year", "current") in attrs:
            if tag != "span" or self.get_starttag_text() != MARKER:
                raise ValueError("year marker must use the exact approved span")
            if not self.in_h1:
                raise ValueError("year marker must be inside H1")
            self.pending = self.source_offset() + len(MARKER)

    def handle_endtag(self, tag):
        if self.pending is not None:
            end = self.source_offset()
            year = self.source[self.pending:end]
            if (tag != "span" or self.source[end:end + 7] != "</span>"
                    or re.fullmatch(r"[0-9]{4}", year) is None):
                raise ValueError("year marker must contain exactly four ASCII digits")
            self.years.append((self.pending, end, year))
            self.pending = None
        if tag == "h1":
            self.in_h1 = False

    def validate(self):
        self.feed(self.source)
        self.close()
        if self.pending is not None:
            raise ValueError("unclosed year marker")
        if self.years and (self.h1_count != 1 or self.in_h1 or len(self.years) != 1):
            raise ValueError("marked page must have one complete H1 and one year marker")


def html_files(root):
    # Skip tooling/dependency directories; inspect HTML throughout the project.
    for directory, dirs, files in os.walk(root):
        dirs[:] = sorted(d for d in dirs if d not in SKIP_DIRS
                         and not (Path(directory) / d).is_symlink())
        for name in sorted(files):
            path = Path(directory) / name
            if path.suffix.lower() == ".html" and not path.is_symlink():
                yield path


def update(root, year, dry_run=False):
    plans = []
    scanned = marker_count = 0
    errors = []
    for path in html_files(root):
        scanned += 1
        relative = path.relative_to(root).as_posix()
        try:
            original = path.read_bytes()
            # Latin-1 maps bytes 1:1, so source offsets and line endings survive.
            parser = GuideYearParser(original.decode("latin-1"))
            parser.validate()
            marker_count += len(parser.years)
            updated = original
            for start, end, old_year in reversed(parser.years):
                updated = updated[:start] + str(year).encode("ascii") + updated[end:]
            plans.append((path, original, updated, parser.years))
        except (OSError, ValueError) as error:
            errors.append(f"{relative}: {error}")

    # Validate the entire scan before writing anything.
    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        print(f"scanned files: {scanned}; marker count: {marker_count}")
        print(f"changed files: 0; unchanged files: {scanned}")
        print("No files written: validation failed.", file=sys.stderr)
        return 1

    changed = 0
    for path, original, updated, years in plans:
        for _, _, old_year in years:
            state = "would change" if dry_run and original != updated else (
                "changed" if original != updated else "unchanged")
            print(f"{path.relative_to(root).as_posix()}: {old_year} -> {year} ({state})")
        if original != updated:
            if not dry_run:
                path.write_bytes(updated)
            changed += 1

    print(f"scanned files: {scanned}")
    print(f"marker count: {marker_count}")
    print(f"changed files: {changed}" + (" (would change; dry run)" if dry_run else ""))
    print(f"unchanged files: {scanned - changed} (includes files without markers)")
    if dry_run:
        print("No files written (dry run).")
    return 0


def parse_year(value):
    if re.fullmatch(r"[0-9]{4}", value) is None or int(value) < 1000:
        raise argparse.ArgumentTypeError("year must be four digits from 1000 to 9999")
    return int(value)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dry-run", action="store_true", help="report without writing files")
    parser.add_argument("--year", type=parse_year, help="override the current Asia/Seoul year")
    args = parser.parse_args()
    year = args.year if args.year is not None else current_kst_year()
    print(f"target year: {year}" + (" (override)" if args.year else " (Asia/Seoul)"))
    try:
        return update(ROOT, year, args.dry_run)
    except OSError as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
